import Head from 'next/head';
import React from 'react';
import Navbar from './Navbar';

const Layout = ({children}: {children: React.ReactNode}) => {
    return (
        <>
            <Head>
                <title>They Could've Met</title>
                <meta name="description" content="Discover who could've met." />
                <link rel="icon" href="/favicon.ico" />
            </Head>
            <Navbar />
            <main>{children}</main>
        </>
    )
}

export default Layout;