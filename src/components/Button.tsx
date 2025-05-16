import { Button } from "@mui/joy";

type KButtonProps = {
    icon: React.ReactNode,
    label: string
};

export default function KButton({icon, label}: KButtonProps) {
    return (
        <Button variant="soft" 
            size="md"
            className="scale-80" 
            startDecorator={icon}
        >
            {label}
        </Button>
    )
}
