from dataclasses import dataclass


@dataclass
class SimStepStats:
    step: int
    living_count: int
    fit_count: int
    fertile_count: int
    poisioned_count: int

    def __add__(self, other: "SimStepStats"):
        return SimStepStats(
            step=self.step,
            living_count=self.living_count + other.living_count,
            fit_count=self.fit_count + other.fit_count,
            fertile_count=self.fertile_count + other.fertile_count,
            poisioned_count=self.poisioned_count + other.poisioned_count,
        )

    @classmethod
    def empty(cls, step: int):
        return cls(
            step=step, living_count=0, fit_count=0, fertile_count=0, poisioned_count=0
        )


def plot_sim_stats(stats: list[SimStepStats], save_path: str | None = None) -> None:
    import matplotlib.pyplot as plt

    if not stats:
        return

    steps = [s.step for s in stats]
    living = [s.living_count for s in stats]
    fit = [s.fit_count for s in stats]
    fertile = [s.fertile_count for s in stats]

    plt.figure(figsize=(10, 5))
    plt.plot(steps, living, label="living_count", linewidth=2)
    plt.plot(steps, fit, label="fit_count", linewidth=2)
    plt.plot(steps, fertile, label="fertile_count", linewidth=2)
    plt.xlabel("Simulation step")
    plt.ylabel("Count")
    plt.title("Simulation counts over time")
    plt.legend(loc="upper right")
    plt.grid(alpha=0.3)
    plt.tight_layout()

    if save_path:
        plt.savefig(save_path)
    else:
        plt.show()

    plt.close()
