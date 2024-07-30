<script lang="ts">
  import { createChartData } from "$lib/scripts/helpers/formatChartData.js";
  import { Line } from "svelte-chartjs";
  export let data;

  const INTERVAL_TYPES = ["1h", "4h", "1d", "1w"] as const;
  type IntervalTypes = (typeof INTERVAL_TYPES)[number];

  $: console.log("data:", data);

  let totalViews = data.total_profile_views;
  // this cast is type safe because data.intervalType has a type in the union
  let interval: IntervalTypes =
    data.intervalType ?? "1h" in INTERVAL_TYPES
      ? (data.intervalType as IntervalTypes)
      : "1h";

  let intervalViews = data.interval_views;
  let chartData1 = createChartData(
    intervalViews.map((interval) => interval[0]),
    "View count",
    intervalViews,
  );
</script>

<div>
  <Line data={chartData1} options={{ responsive: true }} />
</div>
