import type Person from '../utils/person';
import Image from 'next/image'

type Props = {
    person: Person;
    onClick?: (p: Person) => void;
};

const PersonCard = ({ person , onClick}: Props) => {
    return (
        <div>
            <div>
                <Image 
                    src={person.thumbnail_link}
                    height={300}
                    width={200}
                />
            </div>
            <div>
                <p>{person.name}</p>
                <p>Birth Date: {person.birth}</p>
                <p>Death Date: {person.death ? person.death : "Still Alive"}</p>
                <p>{person.description_en}</p>
                <a href={person.wikipedia_link}>Learn more...</a>
            </div>
            {
                (onClick !== undefined) && 
                <button onClick={(e) => {onClick(person);}}>Select</button>    
                
            }
        </div>
    )
}

export default PersonCard;