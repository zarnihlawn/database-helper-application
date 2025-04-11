import type { ReleaseNoteInterface } from "../interface/release-note.interface";

// Release note data
//#region
export const ReleaseNoteData: ReleaseNoteInterface[] = [
    {
        title: 'v0.1.0',
        content: ['Initial release of the application', 'Added support for new feature', 'Fixed bug in feature'],
        date: new Date('2023-01-01')
    },
    {
        title: 'v0.1.1',
        content: ['Added support for new feature', 'Added new feature'],
        date: new Date('2023-02-01')
    },
    {
        title: 'v0.1.2',
        content: ['Fixed bug in feature', 'Nicsu'],
        date: new Date('2023-03-01')
    }
];
//#endregion