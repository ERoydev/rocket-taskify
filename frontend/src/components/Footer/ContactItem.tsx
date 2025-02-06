export default function ContactItem({
    labelText,
    pText,
    svgItem
}) {
    return(
        <div className="flex gap-3 hover:bg-slate-50/[0.05] hover:cursor-pointer p-2 rounded-xl scale-95 transition-shadow hover:scale-100 hover:shadow-lg hover:shadow-gray-700">
            {svgItem}
            <div>
                <label htmlFor="name" className="font-normal text-center font-palanquin text-gray-300 xl:text-[1rem] md:text-[0.8rem] text-[0.7rem]">{labelText}</label>
                <p id="name" className="font-bold text-white xl:text-[1rem] md:text-[0.85rem] text-[0.7rem]">{pText}</p>
            </div>
        </div>

    );
}