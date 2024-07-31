<script lang="ts">
  import { createChartData } from "$lib/scripts/helpers/formatChartData.js";
  import { Line } from "svelte-chartjs";
  import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale,
  } from "chart.js";

  ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale,
  );

  export let data;

  const INTERVAL_TYPES = ["1h", "4h", "1d", "1w"] as const;
  type IntervalTypes = (typeof INTERVAL_TYPES)[number];

  // $: console.log("data:", data);

  // let totalViews = data.total_profile_views;
  // this cast is type safe because data.intervalType has a type in the union
  let interval: IntervalTypes =
    data.intervalType ?? "1h" in INTERVAL_TYPES
      ? (data.intervalType as IntervalTypes)
      : "1h";

  let intervalViews = data.interval_views;
  let intervalFollows = data.interval_follow_requests;
  // sample data
  // views

  /*
  let intervalViews: [Date, number][] = [
    [new Date(2024, 7, 31, 0), 5],
    [new Date(2024, 7, 31, 1), 10],
    [new Date(2024, 7, 31, 2), 4],
    [new Date(2024, 7, 31, 3), 50],
    [new Date(2024, 7, 31, 4), 30],
    [new Date(2024, 7, 31, 5), 6],
    [new Date(2024, 7, 31, 6), 16],
    [new Date(2024, 7, 31, 7), 19],
    [new Date(2024, 7, 31, 8), 25],
    [new Date(2024, 7, 31, 9), 30],
  ];
  */
  // follows
  /*
  let intervalFollows: [Date, number][] = [
    [new Date(2024, 7, 31, 0), 2],
    [new Date(2024, 7, 31, 0), 2],
    [new Date(2024, 7, 31, 1), 3],
    [new Date(2024, 7, 31, 2), 1],
    [new Date(2024, 7, 31, 3), 5],
    [new Date(2024, 7, 31, 4), 6],
    [new Date(2024, 7, 31, 5), 1],
    [new Date(2024, 7, 31, 6), 0],
    [new Date(2024, 7, 31, 7), 3],
    [new Date(2024, 7, 31, 8), 1],
    [new Date(2024, 7, 31, 9), 1],
  ];
  */

  // right now only support 1 hour
  if (interval == "1h") {
    // take the last 24 hours of data
    intervalViews.slice(Math.max(intervalViews.length - 24, 1));
    intervalFollows.slice(Math.max(intervalFollows.length - 24, 1));
  }

  let chartDataViews = createChartData(
    intervalViews.map((interval, index) => intervalViews.length - 1 - index),
    "View count",
    intervalViews,
  );
  let chartDataFollows = createChartData(
    intervalFollows.map(
      (interval, index) => intervalFollows.length - 1 - index,
    ),
    "Follow request count",
    intervalFollows,
  );
</script>

<div>
  <Line
    style="height:500px; width:1400px"
    class="p-4"
    data={chartDataViews}
    options={{
      responsive: false,
      scales: {
        x: { title: { display: true, text: "hours ago", font: { size: 20 } } },
      },
    }}
  />
  <Line
    style="height:500px; width:1400px"
    class="p-4"
    data={chartDataFollows}
    options={{
      responsive: false,
      scales: {
        x: { title: { display: true, text: "hours ago", font: { size: 20 } } },
      },
    }}
  />
</div>
