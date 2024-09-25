const sidebar = [
    {
        text: 'Introduction',
        items: [
            { text: 'Soroban', link: '/introduction' },
            { text: 'Installation', link: '/installation' },
            { text: 'Getting Started', link: '/getting-started' },
            { text: 'Hello World', link: '/hello-world' },
            { text: 'Counter App', link: '/counter' },
        ],
    },
    {
        text: 'Basic',
        items: [
            { text: 'Integers', link: '/integers' },
            { text: 'Boolean', link: '/boolean' },
            { text: 'Strings', link: '/strings' },
            { text: 'Symbol', link: '/symbol' },
            { text: 'Bytes', link: '/bytes' },
            { text: 'Vec', link: '/vec' },
            { text: 'Map', link: '/map' },
            { text: 'Address', link: '/address' },
            { text: 'Structs', link: '/structs' },
            { text: 'Transient Storage', link: '/transient-storage' },
            { text: 'Events', link: '/events' },
            { text: 'Logging', link: '/logging' },
            { text: 'Persistent Storage', link: '/persistent-storage' },
        ],
    },
    {
        text: 'ERC',
        items: [
            { text: 'Token Interface (SEP)', link: '/token-interface' },
            { text: 'Admin Interface(SEP)', link: '/admin-interface' },
            { text: 'Timelock', link: '/timelock' },
        ],
    },
    {
        text: 'Applications',
        items: [
            { text: 'Only Owner', link: '/only-owner' },
            { text: 'Multisig', link: '/multisig' },
            { text: 'Loops', link: '/loops' },
        ],
    },
];

export default sidebar;
