export default function SocialItem({
    imgPath,
    linkName
}: {
    imgPath: string;
    linkName: string;
}) {

    const clickHandler = () => {
        if (linkName == 'Github') {
            window.open('https://github.com/ERoydev/', '_blank');
        } else if (linkName == 'Linkedin') {
            window.open('https://www.linkedin.com/in/emil-roydev-91450b26a/', '_blank');
        } else {
            window.open('https://www.discordapp.com/users/1071352603524472924/', '_blank');
        }
    }
    return(
        <div className="window p-3 hover:bg-slate-800 hover:cursor-pointer scale-90 shadow-none transition-shadow hover:scale-100 hover:shadow-lg hover:shadow-gray-700" onClick={clickHandler}>
            <img src={imgPath} alt="Social Link icon" className="xl:w-[35px]  xl:h-[35px] w-[20px] h-[20px]"/>
        </div>
    );
}