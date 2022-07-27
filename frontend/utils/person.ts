interface Person {
    birth: string;
    death: string | null;
    description_en: string;
    id: number;
    links_to_page: number;
    name: string;
    thumbnail_link: string;
    wikipedia_link: string;
}

export default Person;