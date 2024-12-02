import React, { useState } from 'react';
import parserManager from '../utils/parser';
import Button from '../components/Button';

const asset = {
    0: 'line.png',
    1: 'corner.png',
    8: 'santa.png',
    9: 'socks2.png',
}

const rotation = ['0', '90', '180', '270'];

function Grid({ map, startGrid }) {
    const [cells, setCells] = useState(startGrid);

    const handleClick = (pos) => {
        const i = parseInt(pos[0]);
        const j = parseInt(pos[1]);

        if ((i === 0 && j === 0) || (i === 6 && j === 6)) return;

        const row = [...cells[i]];
        row[j] = (row[j] + 1) % 4;

        const newGrid = [...cells];
        newGrid[i] = row;

        setCells(newGrid);
    };

    const handleSolve = async (currentState, map) => {
        const parsed = parserManager.parseMap(currentState, map);
        const body = JSON.stringify({
            "source": {
                "x": 0,
                "y": 0
            },
            "goal": {
                "x": 6,
                "y": 6
            },
            "grid": parsed
        });
        const response = await fetch('http://localhost:8086/solver',{
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: body
        })
        if(!response.ok) {
            alert('Error');
            return;
        } else {
            const data = await response.json();
            const newGrid = data.grid;
            const decodedGrid = parserManager.resrap(newGrid);
            console.log(decodedGrid);
        }
    }

    return (
        <div className='flex flex-col gap-4'>
            <div className="grid grid-cols-7 grid-rows-7 gap-0 border">
                {cells.map((row, i) => (
                    row.map((cell, j) => {
                        const pos = String(i) + String(j);
                        return (
                            <div key={pos}
                                className='w-12 h-12 md:h-20 sm:w-20 cursor-pointer justify-center flex items-center border box-border'
                                onClick={() => handleClick(pos)}
                            >
                                <img src={`${asset[map[i][j]]}`} alt='line or corner'
                                    style={{ transform: `rotate(${rotation[cell]}deg)` }}
                                    draggable="false"
                                />
                            </div>
                        )
                    })
                ))}
            </div>
            <div className='flex items-center justify-center'>
                <Button variant="tuzas" action={() => handleSolve(cells, map)}>
                    SOLVE IT
                </Button>
            </div>
        </div>
    );
};

export default Grid;
