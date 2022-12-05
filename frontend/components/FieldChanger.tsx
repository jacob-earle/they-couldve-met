import React, {useState} from 'react';

import getConfig from 'next/config';
import { resourceLimits } from 'worker_threads';

const {publicRuntimeConfig} = getConfig();

const apiBaseUrl = publicRuntimeConfig.NEXT_PUBLIC_API_BASE_URL;

type Props = {
    name: string;
    id: number;
}

const FieldChanger = ({name, id}: Props) => {
    const [value, setValue] = useState(name);
    
    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        const updateUrl = encodeURI(apiBaseUrl + id)
        fetch(updateUrl, {
            method: 'post',
            body: JSON.stringify({new_name: value}),
            headers: { 'Content-Type': 'application/json' },
        })
            .then(result => {
                if (result.status === 200) {
                    alert("Field updated successfully.")
                } else {
                    alert("Error updating field.")
                }
            })
            .catch(e => {console.log(e)})
    } 
    
    return (
        <form onSubmit={handleSubmit}>
            <label>
                <input className='textL' type="text" value={value} onChange={e => setValue(e.target.value)}/>
            </label>
            <button type='submit'>Update</button>
        </form>
    );
};

export default FieldChanger;