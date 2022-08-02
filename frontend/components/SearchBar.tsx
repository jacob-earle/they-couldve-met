import PersonCard from "./PersonCard";
import Person from '../utils/person';
import { useState } from "react";
import getConfig from 'next/config';

const {publicRuntimeConfig} = getConfig();

const apiBaseUrl = publicRuntimeConfig.NEXT_PUBLIC_API_BASE_URL;

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
        <input className="textL" type="text" value={searchString} onChange={(e) => {
        setSearchString(e.target.value);
        }}/>
    </label>
    <button className="textL" type="submit">Search</button>
    </form>
    {
        searchResultsList.map((person) => <PersonCard key={person.id} person={person} onClick={onClick}/>)
    }
  </div>)

}

export default SearchBar