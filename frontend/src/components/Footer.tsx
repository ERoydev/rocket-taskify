import ContactItem from "./Footer/ContactItem";
import SocialItem from "./Footer/SocialItem";
import github from "../assets/svg/github.svg";
import linkedin from "../assets/svg/linkedin.svg";
import discord from "../assets/svg/discord.svg";

export default function Footer() {
    const handleMessageButton = () => {
        window.location.href = 'mailto:e.roydev@gmail.com?subject=Contact%20Us&body=Hi%20there,%0D%0A%0D%0AI%20would%20like%20to%20get%20in%20touch%20with%20you%20regarding...%0D%0A%0D%0AThank%20you!';
    };

    return(
        <>
            <div className="max-container flex flex-col items-center mt-10" id="contact">
                <div className="mb-14 2xl:w-[70%] lg:w-[90%]">
                    <h1 className="mb-6 text-white xl:text-[3.4rem] md:text-[2.2rem] text-[1.8rem] text-center">Get in <span className="text-purple-600">Touch</span></h1>
                    <p className="font-normal text-center font-palanquin text-gray-300 leading-normal mx-auto xl:text-lg md:text-[1rem] sm:text-[0.9rem] text-[0.6rem] opacity-[.90]">Drop me a line!.</p>
                </div>

                <div className="sm:flex xl:w-[60%] justify-between mb-16 gap-10 max-sm:space-y-10">
                    <div className="flex gap-3">
                        <ContactItem pText={'Emil Roydev'} labelText={'Name:'} svgItem={<svg className="lg:w-[40px] lg:h-[40px] w-[30px] h-[30px] fill-purple-700" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M224 256A128 128 0 1 0 224 0a128 128 0 1 0 0 256zm-45.7 48C79.8 304 0 383.8 0 482.3C0 498.7 13.3 512 29.7 512H418.3c16.4 0 29.7-13.3 29.7-29.7C448 383.8 368.2 304 269.7 304H178.3z"/></svg>}/>
                    </div>

                    <div className="flex gap-3">
                        <ContactItem pText={'Ruse, Bulgaria'} labelText={'Location:'} svgItem={<svg className="lg:w-[40px] lg:h-[40px] w-[30px] h-[30px] fill-purple-700" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M215.7 499.2C267 435 384 279.4 384 192C384 86 298 0 192 0S0 86 0 192c0 87.4 117 243 168.3 307.2c12.3 15.3 35.1 15.3 47.4 0zM192 128a64 64 0 1 1 0 128 64 64 0 1 1 0-128z"/></svg>}/>
                    </div>

                    <div className="flex gap-3">
                        <ContactItem pText={'+359 87 735 3733'} labelText={'Phone:'} svgItem={<svg className="lg:w-[40px] lg:h-[40px] w-[30px] h-[30px] fill-purple-700" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M164.9 24.6c-7.7-18.6-28-28.5-47.4-23.2l-88 24C12.1 30.2 0 46 0 64C0 311.4 200.6 512 448 512c18 0 33.8-12.1 38.6-29.5l24-88c5.3-19.4-4.6-39.7-23.2-47.4l-96-40c-16.3-6.8-35.2-2.1-46.3 11.6L304.7 368C234.3 334.7 177.3 277.7 144 207.3L193.3 167c13.7-11.2 18.4-30 11.6-46.3l-40-96z"/></svg>} />
                    </div>
                </div>

                <div className="mb-24 flex justify-center">
                    <div className="flex gap-3">
                        <ContactItem pText={'e.roydev@gmail.com'} labelText={'E-mail:'} svgItem={<svg className="lg:w-[40px] lg:h-[40px] w-[30px] h-[30px] fill-purple-700" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M48 64C21.5 64 0 85.5 0 112c0 15.1 7.1 29.3 19.2 38.4L236.8 313.6c11.4 8.5 27 8.5 38.4 0L492.8 150.4c12.1-9.1 19.2-23.3 19.2-38.4c0-26.5-21.5-48-48-48H48zM0 176V384c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V176L294.4 339.2c-22.8 17.1-54 17.1-76.8 0L0 176z"/></svg>}/>

                    </div>
                </div>
            </div>

        
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