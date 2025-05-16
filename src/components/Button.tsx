type ButtonProps = {
    icon: string,
    name: string,
    onPress?: () => void
};

export default function Button({icon, name, onPress}: ButtonProps): JSX.Element  {
    return (
        <button onClick={() => {
            if (onPress)
                onPress();
        }} className="flex flex-col items-center justify-center p-2 scale-75">
            <img src={`${icon}`} className="w-5 mb-1"></img>
            <p className="text-center text-sm">{name}</p>
        </button>
    )
}
