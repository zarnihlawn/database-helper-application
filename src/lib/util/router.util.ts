import { goto } from '$app/navigation';

export function goToRoute(path: string) {
    goto(path);
}
