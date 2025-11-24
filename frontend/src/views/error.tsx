import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle, CardFooter, CardContent, CardDescription } from "@/components/ui/card";
import { BugIcon } from "lucide-react";

export function ErrorView({ error, onRestoreClick }: { error: Error | string; onRestoreClick?: () => void }) {
    return (
        <div className="grid h-full min-h-[inherit] w-full place-items-center">
            <Card className="bg-destructive/5 border-destructive/10">
                <CardHeader>
                    <CardTitle className="text-destructive">
                        <BugIcon className="mr-2 mb-1 inline" size={20} />
                        An unexpected error occurred
                    </CardTitle>
                    <CardDescription>
                        If this message keeps appearing, please report it to the developers or contact your system
                        administrator to resolve the issue.
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <code className="bg-destructive/10 border-destructive/20 min-h-64 overflow-scroll rounded border px-1 py-0.5 text-sm wrap-break-word whitespace-pre-wrap">
                        {typeof error === "string" ? error : error.message}
                    </code>
                </CardContent>
                <CardFooter>
                    <Button onClick={onRestoreClick} variant="destructive-outline" className="w-full">
                        Close
                    </Button>
                </CardFooter>
            </Card>
        </div>
    );
}
