import { createResource, type Component, Show } from "solid-js";
import * as Plot from "@observablehq/plot";
import Chart from "./Chart";

const fetchData = async (bucket: string) =>
  (await fetch(`http://localhost:5005/metrics/${bucket}`)).json();

const App: Component = () => {
  const [data] = createResource("living_room", fetchData);

  return (
    <div class="flex flex-col gap-4 p-8">
      <p class="text-2xl">Living room</p>
      <Show when={!data.loading}>
        <div class="grid grid-cols-2 gap-8">
          <Chart
            options={{
              y: { grid: true, label: "co2" },
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
