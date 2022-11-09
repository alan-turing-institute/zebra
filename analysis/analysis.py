"""Functions for analysis and plotting of simulation states."""
# pylint: disable=invalid-name
import json
import numpy as np
import pandas as pd
import toml
import matplotlib.pyplot as plt
import matplotlib.animation as anim


def load_sim_states(fname):
    """Load states during simulation."""
    with open(fname, "r", encoding="utf-8") as f:
        states = []
        for line in f:
            states.append(json.loads(line))
        return states


def load_toml(fname="../zebra.toml"):
    """Load config toml."""
    with open(fname, "r", encoding="utf-8") as f:
        parsed_toml = toml.loads(f.read())
    return parsed_toml


def get_animation(states, zebra_toml, **kwargs):  # pylint: disable=too-many-locals
    """Function to create animation."""
    fig = plt.figure(figsize=(10, 4))
    ax1 = fig.add_subplot(1, 1, 1)
    frame_limit = kwargs.get("frame_limit", 2000)
    gap = kwargs.get("gap", 0.1)
    width = kwargs.get("width", 0.4)
    length = kwargs.get("length", 10)
    n_stripes = kwargs.get("n_stripes", 7)
    road_width = kwargs.get("road_width", 0.15)

    max_length = zebra_toml["road_length"] + 1

    ax1.set_xlim(0, max_length)
    min_y, max_y = -1, 1
    ax1.set_ylim(min_y, max_y)
    ax1.spines["right"].set_visible(False)
    ax1.spines["top"].set_visible(False)
    ax1.spines["left"].set_visible(False)
    ax1.tick_params(left=False)
    ax1.tick_params(labelleft=False)

    (ped_scat,) = ax1.plot(
        [], [], color="dodgerblue", marker="o", markersize=9, ls="None"
    )
    (veh_scat_up,) = ax1.plot([], [], marker=">", color="firebrick", ms=5, ls="None")
    (veh_scat_down,) = ax1.plot([], [], marker="<", color="purple", ms=5, ls="None")

    time_text = ax1.text(
        0, 1, "", ha="left", va="bottom", transform=ax1.transAxes, fontsize="large"
    )

    # vlines for 100m intervals
    ax1.vlines(
        np.arange(0, max_length, 100),
        min_y,
        max_y,
        ls=":",
        color="grey",
        lw=0.5,
        zorder=-1,
    )

    # hlines for zebra stripes
    for crossing in zebra_toml["zebra_crossings"]:
        ax1.hlines(
            np.linspace(-width, width, n_stripes),
            crossing - length / 2,
            crossing + length / 2,
            color="k",
            lw=5,
            zorder=-4,
        )

    def convert_millis(ms):
        secs = int((ms / 1000)) % 60
        mins = int(ms / (1000 * 60)) % 60
        hrs = int(ms / (1000 * 60 * 60)) % 24
        return hrs, mins, secs

    def init():
        (ped_scat,) = ax1.plot(
            [], [], color="dodgerblue", marker="o", markersize=9, ls="None"
        )
        (veh_scat_up,) = ax1.plot(
            [], [], marker=">", color="firebrick", ms=5, ls="None"
        )
        (veh_scat_down,) = ax1.plot([], [], marker="<", color="purple", ms=5, ls="None")
        time_text = ax1.text(
            0, 1, "", ha="left", va="bottom", transform=ax1.transAxes, fontsize="large"
        )
        return ped_scat, veh_scat_up, veh_scat_down, time_text

    def animate(i):
        state = states[i]
        timestamp = int(state["timestamp"])
        hrs, mins, secs = convert_millis(timestamp)
        ped_xs = [
            zebra_toml["zebra_crossings"][int(ped["location"])]
            for ped in state["pedestrians"]
        ]
        ped_ys = []
        counts = {}
        for el in ped_xs:
            if el in counts:
                counts[el] += gap
            else:
                counts[el] = gap + width
            ped_ys.append(counts[el])

        veh_xs_up = [
            float(vehicle["position"])
            for vehicle in state["vehicles"]
            if vehicle["direction"] == "Up"
        ]
        veh_ys_up = len(veh_xs_up) * [road_width / 2]
        veh_xs_down = [
            zebra_toml["road_length"] - float(vehicle["position"])
            for vehicle in state["vehicles"]
            if vehicle["direction"] == "Down"
        ]
        veh_ys_down = len(veh_xs_down) * [-road_width / 2]

        # Update plots
        ped_scat.set_data(ped_xs, ped_ys)
        veh_scat_up.set_data(veh_xs_up, veh_ys_up)
        veh_scat_down.set_data(veh_xs_down, veh_ys_down)
        time_text.set_text(f"Time: {hrs:02.0f}h{mins:02.0f}m{secs:02.0f}s")
        return (ped_scat, veh_scat_up, veh_scat_down, time_text)

    return anim.FuncAnimation(
        fig,
        animate,
        init_func=init,
        frames=np.arange(0, min(frame_limit, len(states))),
        repeat=False,
        blit=True,
        interval=100,
    )


def display_and_save_animation(states, zebra_toml, animation_file, **kwargs):
    """Make animation and save as gif."""

    # Pillow writer for gif
    pillow_writer = plt.matplotlib.animation.PillowWriter(fps=15)

    # Get animation
    animation = get_animation(states, zebra_toml, **kwargs)
    html5 = animation.to_html5_video()

    if kwargs.get("write", True):
        # Save as gif
        animation.save(animation_file, dpi=100, writer=pillow_writer)

    return html5


def get_vehicle_arrival_and_exits(states):
    """
    Gets arrival, exit and transit times for each vehicle as a pandas df.

    Args:
        states: list of json states during simulation.

    Returns:
        df: Dataframe of arrival, exit and transit times per vehicle.
    """
    # Dicts to store times
    vehicle_arrivals = {}
    vehicle_exits = {}

    # Loop over states
    for state in states:
        # Current time
        timestamp = state["timestamp"]

        # Current vehicles
        current_veh = set()

        # Loop over vehicles
        for vehicle in state["vehicles"]:
            vid = vehicle["id"]
            # Add vehicle ID to current set
            current_veh.add(vid)

            # If not already arrived, add time
            if vid not in vehicle_arrivals:
                vehicle_arrivals[vid] = int(timestamp) / 1000

        # Loop over vehicles already arrived
        for vid in vehicle_arrivals:
            # If not in current vehicles and not previously exited
            if vid not in current_veh and vid not in vehicle_exits:
                # Record exit time
                vehicle_exits[vid] = int(timestamp) / 1000

    # Make df from dict
    df = (
        pd.DataFrame.from_dict([vehicle_arrivals])
        .T.rename(columns={0: "arrival"})
        .join(pd.DataFrame.from_dict([vehicle_exits]).T.rename(columns={0: "exit"}))
    )
    # Get transit time in ms
    df["transit"] = df["exit"] - df["arrival"]

    return df
