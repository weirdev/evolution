from dataclasses import dataclass


@dataclass
class SimStepStats:
    step: int
    living_count: int
    fit_count: int
    fertile_count: int
    poisoned_count: int

    def __add__(self, other: "SimStepStats"):
        return SimStepStats(
            step=self.step,
            living_count=self.living_count + other.living_count,
            fit_count=self.fit_count + other.fit_count,
            fertile_count=self.fertile_count + other.fertile_count,
            poisoned_count=self.poisoned_count + other.poisoned_count,
        )

    @classmethod
    def empty(cls, step: int):
        return cls(
            step=step, living_count=0, fit_count=0, fertile_count=0, poisoned_count=0
        )


def plot_sim_stats(stats: list[SimStepStats], save_path: str | None = None) -> None:
    import matplotlib.pyplot as plt

    if not stats:
        return

    steps = [s.step for s in stats]
    living = [s.living_count for s in stats]
    fit = [s.fit_count for s in stats]
    fertile = [s.fertile_count for s in stats]
    poisoned = [s.poisoned_count for s in stats]

    # Use a left y-axis for counts and a right (twin) y-axis for percent
    fig, ax = plt.subplots(figsize=(10, 5))
    ax.plot(steps, living, label="living_count", linewidth=2)
    ax.plot(steps, fit, label="fit_count", linewidth=2)
    ax.plot(steps, fertile, label="fertile_count", linewidth=2)

    # Compute percent poisoned (poisoned / living) and plot on the right axis
    poisoned_pct = [ (p / l * 100) if l > 0 else 0 for p, l in zip(poisoned, living) ]
    ax2 = ax.twinx()
    ax2.plot(steps, poisoned_pct, label="% poisoned", color="C3", linewidth=2)
    ax2.set_ylabel("% poisoned", color="C3")
    ax2.tick_params(axis="y", colors="C3")
    ax2.set_ylim(0, 100)
    ax.set_xlabel("Simulation step")
    ax.set_ylabel("Count")
    ax.set_title("Simulation counts over time")
    # Combine legends from both axes
    lines, labels = ax.get_legend_handles_labels()
    lines2, labels2 = ax2.get_legend_handles_labels()
    ax.legend(lines + lines2, labels + labels2, loc="upper right")
    ax.grid(alpha=0.3)
    fig.tight_layout()

    if save_path:
        fig.savefig(save_path)
    else:
        plt.show()

    plt.close(fig)
