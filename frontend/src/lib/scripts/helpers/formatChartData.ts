export type TDataset = {
  label: string;
  data: number[];
} & unknown;

export type TChartGraph = {
  labels: string[] | number[] | Date[];
  datasets: TDataset[];
};

export const createChartData = (
  labels: Date[],
  graphLabel: string,
  rawInterval: [Date, number][],
): TChartGraph => {
  return {
    labels,
    datasets: [
      {
        label: graphLabel,
        data: rawInterval.map((el) => el[1]),
      },
    ],
  };
};
