import { ViewButton } from "@/components/view-button";
import { BackArrowIcon } from "@/components/icons";
import type { ViewLocation } from "@/views";
import { ViewCard } from "@/components/view-card";

export function HelpSupportView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    return (
        <div className="grid min-h-[inherit] grid-cols-2 grid-rows-2 gap-4 p-4 max-sm:flex max-sm:flex-col max-sm:items-center max-sm:justify-start">
            <ViewButton
                className="col-start-1 row-span-2 row-start-1"
                title="Back to help"
                description="Please try to find the answer to your question in the documentation first."
                icon={<BackArrowIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onLocationSwitch("help")}
            />
            <ViewCard className="col-start-2 row-span-2 row-start-1 justify-start">
                <span className="text-2xl font-semibold">Contact Information</span>
                <span className="text-muted-foreground text-center text-sm text-pretty">
                    Please note that we are often unable to respond immediately or provide assistance in person due to
                    class, work, or personal commitments.
                </span>
                <table className="mt-2 w-full text-sm">
                    <tbody>
                        <tr className="border-muted-foreground border-b">
                            <td className="py-2 align-top">Christian</td>
                            <td className="px-2 py-2">
                                christian.fuchte@grb-online.net <br />
                                <span className="text-muted-foreground">+49 171 1234567</span>
                            </td>
                        </tr>
                        <tr>
                            <td className="py-2 align-top">John</td>
                            <td className="px-2 py-2">
                                john.doe@teschnik.de <br />
                                <span className="text-muted-foreground">+49 171 1234567</span>
                            </td>
                        </tr>
                    </tbody>
                </table>
                <span className="text-muted-foreground border-muted-foreground w-full p-2 text-center text-sm text-pretty">
                    We&apos;d prefer to be contacted via email as long as possible, as we are often unable to answer
                    calls.
                </span>
            </ViewCard>
        </div>
    );
}
