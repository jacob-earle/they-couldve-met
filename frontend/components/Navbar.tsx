import Link from 'next/link'
import styles from '../styles/Navbar.module.css'

const Navbar = () => {
    return (
        <nav className={styles.navBar}>
            <div className={styles.titleContainer}>
                <h1 className={styles.heading}>They Could've Met</h1>
            </div>
            <div className={styles.linkContainer}>
                <Link href="/" >Home</Link>
                <Link href="/about" >About</Link>
            </div>
        </nav>
    )
}

export default Navbar;