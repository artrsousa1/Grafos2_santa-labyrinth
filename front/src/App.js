import React from "react";
import Grid from "./components/grid";

function App() {

  return (
    <div className="flex flex-col min-h-screen items-center justify-center gap-8">
      <h1 className="text-3xl md:text-6xl font-bold">Santa Labyrinth</h1>
      <div className="flex relative">
        <Grid />
        <img draggable="false" src="santa.png" alt="Santa" className="h-10 w-10 md:h-24 md:w-24 absolute top-0 left-0" />
        <img draggable="false" src="socks.png" alt="Santa" className="h-10 w-10 md:h-24 md:w-24 absolute bottom-0 right-0"/>
      </div>
    </div>
  );
}

export default App;
