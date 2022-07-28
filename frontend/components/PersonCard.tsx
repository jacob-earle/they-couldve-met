import type Person from '../utils/person';
import Image from 'next/image'
import styles from "../styles/PersonCard.module.css"

type Props = {
    person: Person;
    onClick?: (p: Person) => void;
};

const PersonCard = ({ person , onClick}: Props) => {
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
                <p className='textL'><strong>{person.name}</strong></p>
                <p>Birth Date: {person.birth}</p>
                <p>Death Date: {person.death ? person.death : "Still Alive"}</p>
                <p>{truncatedDescription} (<a href={person.wikipedia_link}>Learn more</a>)
</p>
            </div>
            {
                (onClick !== undefined) && 
                <button onClick={(e) => {onClick(person);}}>Select</button>    
                
            }
        </div>
    )
}

export default PersonCard;