import React, { createContext, useState, useContext, ReactNode } from 'react';

export enum LanguageMode {
    ENGLISH_MODE = 'en',
    KOREAN_MODE = 'kr',
}

type LanguageContextType = {
    language: LanguageMode;
    setLanguage: (language: LanguageMode) => void;
};

export function toLanguageMode(value: string): LanguageMode {
    switch (value) {
        case LanguageMode.ENGLISH_MODE:
            return LanguageMode.ENGLISH_MODE;
        case LanguageMode.KOREAN_MODE:
            return LanguageMode.KOREAN_MODE;
        default:
            return LanguageMode.ENGLISH_MODE;
    }
}

const LANGUAGE_MODE_SAVE_NAME = 'languageMode';
const LanguageContext = createContext<LanguageContextType | undefined>(undefined);

interface LanguageProviderProps {
    children: ReactNode;
}

export const LanguageProvider: React.FC<LanguageProviderProps> = ({ children }) => {
    const [language, setLanguage] = useState<LanguageMode>(() => {
        return toLanguageMode(localStorage.getItem(LANGUAGE_MODE_SAVE_NAME) || LanguageMode.ENGLISH_MODE);
    });

    return (
        <LanguageContext.Provider value={{ language, setLanguage }}>
            {children}
        </LanguageContext.Provider>
    );
};

export const useLanguage = (): LanguageContextType => {
    console.log(LanguageContext);
    const context = useContext(LanguageContext);
    if (!context) {
        throw new Error('useLanguage must be used within a LanguageProvider');
    }
    return context;
};