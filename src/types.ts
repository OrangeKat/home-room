export interface DaylightData {
    sunrise: Date;
    sunset: Date;
}

export interface WeatherData {
    temperature: number;
    description: string;
    image: string;
}

export interface Task {
    id: number;
    title: string;
    date: string;
    time: string;
    isUrgent: boolean;
    completed: boolean;
}

export interface Habit {
    id: number;
    title: string;
    created_at?: string;
}