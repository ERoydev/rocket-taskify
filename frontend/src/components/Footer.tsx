import SocialItem from "./Footer/SocialItem";
import github from "../assets/svg/github.svg";
import linkedin from "../assets/svg/linkedin.svg";
import discord from "../assets/svg/discord.svg";

export default function Footer() {
    return(
        <>
            <div className="flex justify-center items-center gap-10 py-5">
                <p className="text-white-clr md:text-[0.9rem] text-[0.6rem] font-normal text-white text-bold">Copyright &copy; 2025 Emil Roydev</p>

                <div className="flex gap-3">
                    <SocialItem imgPath={github} linkName={'Github'} />
                    <SocialItem imgPath={linkedin} linkName={'Linkedin'} />
                    <SocialItem imgPath={discord} linkName={'Discord'} />
                </div>
            </div>

        </>
    );
}