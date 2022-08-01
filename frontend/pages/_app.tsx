import '../styles/globals.css'
import type { AppProps } from 'next/app'
import Head from 'next/head'

function MyApp({ Component, pageProps }: AppProps) {
  
  return <>
            <Head>
                <title>They Could've Met</title>
                <meta name="description" content="Discover who could've met." />
                <link rel="shortcut icon" href="/favicon.ico" />
            </Head>
            <Component {...pageProps} /> 
          </>
}

export default MyApp
