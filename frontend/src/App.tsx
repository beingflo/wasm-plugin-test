import { createResource, type Component, Show } from "solid-js";
import * as Plot from "@observablehq/plot";
import Chart from "./Chart";

const fetchData = async (bucket: string) =>
  (await fetch(`http://localhost:5005/metrics/${bucket}`)).json();

const App: Component = () => {
  const [data] = createResource("living_room", fetchData);

  return (
    <div class="flex flex-col gap-4 p-8">
      <p class="text-4xl font-bold">Charts</p>
      <Show when={!data.loading}>
        <div class="flex flex-row gap-16">
          <Chart
            options={{
              y: { grid: true, label: "co2" },
              width: 800,
              marks: [
                Plot.lineY(
                  data()?.map((d) => ({ ...d, date: new Date(d.date) })),
                  { x: "date", y: (d) => d.data.co2 }
                ),
              ],
            }}
          />
          <Chart
            options={{
              y: { grid: true, label: "temperature" },
              width: 800,
              marks: [
                Plot.lineY(
                  data()?.map((d) => ({ ...d, date: new Date(d.date) })),
                  { x: "date", y: (d) => d.data.temperature }
                ),
              ],
            }}
          />
        </div>
      </Show>
    </div>
  );
};

export default App;
