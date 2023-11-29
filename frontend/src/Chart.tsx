import { type Component, onMount } from "solid-js";
import * as Plot from "@observablehq/plot";

export type ChartProps = {
  options: Plot.PlotOptions;
};

const Chart: Component<ChartProps> = (props) => {
  let div;

  onMount(() => {
    const plot = Plot.plot(props.options);

    div?.append(plot);
  });

  return <div ref={div} />;
};

export default Chart;
