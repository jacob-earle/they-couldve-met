import type { NextPage } from 'next'
import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'
import type Person from '../utils/person'
import React, { useState, useEffect } from 'react'
import PersonCard from '../components/PersonCard'
import SearchBar from '../components/SearchBar'

const apiBaseUrl = "http://localhost:5000/api/"

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
    <div>
      <Head>
        <title>They Could've Met</title>
        <meta name="description" content="Discover who could've met." />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <h1>They Could've Met</h1>
        <h2>Step 1: Select a person</h2>
        {
          (selectedPerson !== null) ?
          <div>
            <p>You Have Selected:</p>
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
          {selectedPerson !== null && <p>{selectedPerson.name} could have met:</p>}          
          <ul>
            {
              resultsList.map((person) => <li key={person.id}><PersonCard person={person}/></li>)
            }
          </ul>
        </div>
      </main>
    </div>
  )
}

export default Home
