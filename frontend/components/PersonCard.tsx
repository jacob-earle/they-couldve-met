import type Person from '../utils/person';
import Image from 'next/image'
import FieldChanger from './FieldChanger';
import styles from "../styles/PersonCard.module.css"

type Props = {
    person: Person;
    editable?: boolean;
    buttonProps?: {
        onClick: (p: Person) => void;
        buttonText: string;
    }
};

const PersonCard = ({ person , editable, buttonProps}: Props) => {

    const truncatedDescription = person.description_en.length <= 200 
        ? person.description_en 
        : (person.description_en.substring(0, 200) + "...")

    return (
        <div className={styles.cardContainer}>
            <div>
                <Image 
                    src={person.thumbnail_link}
                    height={300}
                    width={200}
                    alt={`Picture of ${person.name}`}
                    placeholder = "empty"
                />
            </div>
            <div>
                {
                    (editable === true) 
                    ? <FieldChanger id={person.id} name={person.name} />
                    : <p className='textL'><strong>{person.name}</strong></p>

                }
                <p>Birth Date: {person.birth}</p>
                <p>Death Date: {person.death ? person.death : "Still Alive"}</p>
                <p>{truncatedDescription} (<a href={person.wikipedia_link}>Learn more</a>)
</p>
            </div>
            {
                (buttonProps !== undefined) && 
                <button onClick={(e) => {buttonProps.onClick(person);}}>{buttonProps.buttonText}</button>    
                
            }
        </div>
    )
}

export default PersonCard;