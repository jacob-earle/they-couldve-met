import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'
import type Person from '../utils/person'
import React, { useState, useEffect } from 'react'
import PersonCard from '../components/PersonCard'
import SearchBar from '../components/SearchBar'
import Navbar from '../components/Navbar';
import Layout from '../components/Layout';
import getConfig from 'next/config';

const {publicRuntimeConfig} = getConfig();

const apiBaseUrl = publicRuntimeConfig.NEXT_PUBLIC_API_BASE_URL;

const Home: NextPage = () => {
  const [resultsList, setResultsList] = useState<Array<Person>>([])
  const [selectedPerson, setSelectedPerson] = useState<Person | null>(null)
  
  // Get results from backend for person with id number
  const getResultsForId = (id: number) => {
    const searchUrl = encodeURI(apiBaseUrl + "match/" + id);
    return fetch(searchUrl)
      .then(response => {console.log(response); return response.json()})
      .then(results => setResultsList(results))
      .catch(e => [])
  }
  
  const handleSelectedPerson = (p: Person) => {
    setSelectedPerson(p);
    getResultsForId(p.id);
  }
  

  
  return (
    <Layout>
      <h2>Step 1: Select a person</h2>
      {
        (selectedPerson !== null) ?
        <div>
          <p className='textXL'>You Have Selected:</p>
          <PersonCard person={selectedPerson} />
          <button onClick={
            (e) => {
              setResultsList([]);
              setSelectedPerson(null);
            }
          }>Start Over</button>
        </div> :
        <SearchBar onClick={handleSelectedPerson} />
      }
      <div>
        <h2>Step 2: See who they could've met!</h2>
        {selectedPerson !== null && <p className='textXL'><strong>{selectedPerson.name}</ strong> could have met:</p>}          
          {
            resultsList.map((person) => <PersonCard key={person.id} person={person}/>)
          }
      </div>
    </Layout>
  )
}

export default Home
