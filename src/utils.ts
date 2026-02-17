import type { DaylightData } from "@/types";

export const isDaytime = (daylightData: DaylightData | null | undefined, time: Date) => {
    if (!daylightData) {
        const hours = time.getHours();
        return hours >= 6 && hours < 18;
    }
    
    const now = time.getTime();
    const sunrise = daylightData.sunrise.getTime();
    const sunset = daylightData.sunset.getTime();
    
    return now >= sunrise && now < sunset;
};