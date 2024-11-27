import React, { useState } from 'react';

const Grid = () => {
    const [ cells, setCells ] = useState(Array(7).fill(Array(7).fill(0)));

    const handleClick = (row, col) => {
        const newCells = cells.map((r, i) =>
            r.map((cell, j) => (i === row && j === col ? (cell + 1) % 4 : cell))
        );
        setCells(newCells);
    };

    return (
        <div className="flex justify-center items-center border">
            <div className='divide-y divide-black'>
                {cells.map((row, rowIndex) => (
                    <div key={rowIndex} className="flex divide-x divide-black">
                        {row.map((cell, colIndex) => (
                            <button
                                key={colIndex}
                                onClick={() => handleClick(rowIndex, colIndex)}
                                className="bg-primary text-center h-10 w-10 md:h-24 md:w-24"
                            >
                                {cell}
                            </button>
                        ))}
                    </div>
                ))}
            </div>
        </div>
    );
};


export default Grid;