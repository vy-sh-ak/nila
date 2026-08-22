export function getModelProvider(url: string): string {
    const url_lower = url.toLowerCase();
    if (url_lower.includes("nvidia")) {
        return "nvidia"
    } else if (url_lower.includes("openrouter")) {
        return "openrouter"
    }
    return ""
}