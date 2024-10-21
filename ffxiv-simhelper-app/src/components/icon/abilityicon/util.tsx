// remove skill hash code from the skill name
export const parseSkillName = (skillPath: string) => {
    if (skillPath.startsWith('data')) {
        return 'pot';
    }

    if (!skillPath.includes('-')) {
        return skillPath
    }

    let splittedString = skillPath.split("-").slice(0, -1);


    let skillName = splittedString[0];


    for (const token of splittedString.slice(1)) {
        skillName += ("-" + token);
    }

    return skillName;
}