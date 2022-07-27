import PersonCard from "./PersonCard";
import Person from '../utils/person';
import { useState } from "react";

const apiBaseUrl = "http://localhost:5000/api/"

type Props = {
  onClick: (p: Person) => void;
}

const SearchBar = ({onClick}: Props) => {
  const [searchString, setSearchString] = useState<string>("")
  const [searchResultsList, setSearchResultsList] = useState<Array<Person>>([])
    
  const fetchSearchResultsList = (s: string): Promise<Array<Person>> => {
    const searchUrl = encodeURI(apiBaseUrl + "search?q=" + s);
    return fetch(searchUrl)
      .then(response => {return response.json()})
      .catch(e => [])
  }
  
  const handleSearchSubmit = (event: React.FormEvent<HTMLFormElement>): void => {
    event.preventDefault();

    fetchSearchResultsList(searchString)
      .then(results => {
        setSearchResultsList(results);
      });
  }
    
  return (
  <div>
    <form onSubmit={handleSearchSubmit}>
    <label>
        <input type="text" value={searchString} onChange={(e) => {
        setSearchString(e.target.value);
        }}/>
    </label>
    <button type="submit">Search</button>
    </form>
    <ul>
    {
        searchResultsList.map((person) => <li key={person.id}><PersonCard person={person} onClick={onClick}/></li>)
    }
    </ul>
  </div>)

}

export default SearchBar