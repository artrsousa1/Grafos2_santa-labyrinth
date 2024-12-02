import React, { useState } from "react";
import Grid from "./components/Grid";
import Button from "./components/Button";
import { SquareArrowOutUpRight } from 'lucide-react';
import { initialState, maps } from "./utils/maps";

function App() {

  const [pos, setPos] = useState(0);

  const handleNext = () => {
    setPos((pos + 1) % maps.length);
  }

  const handlePrev = () => {
    setPos((pos - 1 + maps.length) % maps.length);
  }

  const goToGithub = () => {
    window.open("https://github.com/projeto-de-algoritmos-2024/Grafos2_santa-labyrinth", "_blank")
  }

  return (
    <>
      <div className="bg-iagorrr flex shadow-xl items-center justify-center py-4">
        <h1 className="text-4xl md:text-6xl font-bold">Santa Labyrinth</h1>
        <Button variant="iagorrr" className="absolute right-10 gap-4 flex invisible md:visible" action={goToGithub}>
          <SquareArrowOutUpRight size={24} />
          <span className="font-medium">GitHub</span>
        </Button>    
      </div>
      <div className="flex flex-col items-center justify-center gap-8 mt-8">
        <div className="flex items-center justify-center gap-4">
          <Button variant="tuzas" action={handlePrev}>PREV</Button>
          <Button variant="tuzas" action={handleNext}>NEXT</Button>
        </div>
        <div className="flex items-start relative">
          <Grid startGrid={initialState[pos]} map={maps[pos]}/>
        </div>
      </div>
    </>
  );
}

export default App;
