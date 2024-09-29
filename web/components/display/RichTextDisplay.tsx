import styles from "./RichTestDisplay.module.scss"

interface RichTextDisplayProps {
    content: string;
    fullSize: boolean;
}

const RichTextDisplay = ({content, fullSize}: RichTextDisplayProps) => {


    return (
        <div className={fullSize ? styles.fullSize : styles.capped} dangerouslySetInnerHTML={{
            __html: content
        }}>
        </div>
    )
}

export default RichTextDisplay;
