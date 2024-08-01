export type TDataset = {
  label: string;
  fill: boolean;
  data: number[];
  borderColor: string;
  backgroundColor: string;
} & unknown;

export type TChartGraph = {
  labels: string[] | number[] | Date[];
  datasets: TDataset[];
};

export const createChartData = (
  labels: number[],
  graphLabel: string,
  rawInterval: [Date, number][],
): TChartGraph => {
  return {
    labels,
    datasets: [
      {
        fill: true,
        label: graphLabel,
        data: rawInterval.map((el) => el[1]),
        borderColor: "rgb(50, 205,50)",
        backgroundColor: "rgb(50, 205,50)",
      },
    ],
  };
};
