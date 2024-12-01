import React from "react";

const variants = {
    tuzas: 'bg-accent-hover hover:bg-accent text-iagorrr font-bold py-2 px-8 md:text-3xl rounded text-md',
    iagorrr: 'border-2 py-2 px-6 md:px-4 rounded-lg md:text-xl text-md font-extrabold hover:bg-primary',
}

function Button({ children, action, variant, className }) {
    return (
        <button onClick={action} className={`${variants[variant]} ${className}`}>
            {children}
        </button>
    );
}

export default Button;