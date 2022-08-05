import PersonCard from "../components/PersonCard";
import Person from '../utils/person';
import { useState, useEffect } from "react";
import getConfig from 'next/config';

const {publicRuntimeConfig} = getConfig();

const apiBaseUrl = publicRuntimeConfig.NEXT_PUBLIC_API_BASE_URL;

const Admin = () => {
  // helper function for getting all people
  const fetchAllPeople = (): Promise<Array<Person>> => {
    const searchUrl = encodeURI(apiBaseUrl + "all");
    return fetch(searchUrl)
      .then(response => {return response.json()})
      .catch(e => [])
  };
  
  const [allPeople, setAllPeople] = useState<Array<Person>>([]);
  
  // get all people
  useEffect(() => {
    fetchAllPeople()
      .then(results => {
        setAllPeople(results);
      })
  }, [])
  
  // helper function for deleting a person from a person card
  const deletePerson = (p: Person) => {
    const deleteUrl = encodeURI(apiBaseUrl + p.id);
    fetch(deleteUrl, {
      method: "delete"
    })
      .then(response => {
        if (response.status === 200) {
          alert("Person successfully deleted.")
        } else {
          alert("Error deleting person.")
        }
      })
      .catch(e => console.log(e));
  }

  return (
    <div>
      <h1>Admin Page</h1>
      {
        allPeople.map((person) => <PersonCard key={person.id} person={person} editable={true} buttonProps={{onClick: deletePerson, buttonText: "Delete"}}/>)   
      }
    </div>
  );
}

export default Admin;