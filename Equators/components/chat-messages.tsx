"use client"

import type React from "react"

import { useState } from "react"
import type { Message } from "@/types"
import { Avatar } from "@/components/ui/avatar"
import { ScrollArea } from "@/components/ui/scroll-area"
import { cn } from "@/lib/utils"
import { User, Copy, Check, FileText, ImageIcon, Terminal } from "lucide-react"
import { Button } from "@/components/ui/button"
import ReactMarkdown from "react-markdown"

interface ChatMessagesProps {
  messages: Message[]
  isProcessing: boolean
  messagesEndRef: React.RefObject<HTMLDivElement>
}

export function ChatMessages({ messages, isProcessing, messagesEndRef }: ChatMessagesProps) {
  const [copiedMessageId, setCopiedMessageId] = useState<string | null>(null)

  const copyToClipboard = (text: string, messageId: string) => {
    navigator.clipboard.writeText(text)
    setCopiedMessageId(messageId)
    setTimeout(() => setCopiedMessageId(null), 2000)
  }

  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp)
    return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })
  }

  return (
    <ScrollArea className="h-full pb-10">
      <div className="flex flex-col gap-6 p-4 pb-20">
        {messages.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full min-h-[60vh]">
            <div className="flex h-20 w-20 items-center justify-center rounded-full bg-primary/10 mb-4">
              <Terminal className="h-10 w-10 text-primary" />
            </div>
            <h3 className="text-xl font-semibold">Google Cloud Command Generator</h3>
            <p className="text-muted-foreground text-center max-w-md mt-2">
              Describe what you want to do in Google Cloud using natural language, and I'll generate the appropriate
              gcloud command for you.
            </p>
            <div className="mt-6 bg-secondary p-4 rounded-md max-w-md">
              <p className="text-sm font-medium mb-2">Example prompts:</p>
              <ul className="text-sm space-y-2">
                <li>"Create a new VM instance in us-central1 with 2 CPUs"</li>
                <li>"List all my Kubernetes clusters"</li>
                <li>"Deploy a new app to App Engine"</li>
                <li>"Show me my billing account information"</li>
              </ul>
            </div>
          </div>
        ) : (
          messages.map((message) => (
            <div
              key={message.id}
              className={cn("flex gap-3 relative group", message.role === "assistant" ? "items-start" : "items-start")}
            >
              <Avatar
                className={cn("h-8 w-8 rounded-md", message.role === "assistant" ? "bg-primary/10" : "bg-secondary")}
              >
                {message.role === "assistant" ? (
                  <Terminal className="h-5 w-5 text-primary" />
                ) : (
                  <User className="h-5 w-5" />
                )}
              </Avatar>

              <div className="flex-1 space-y-2">
                <div className="flex items-center gap-2">
                  <span className="font-medium">{message.role === "assistant" ? "GCloud Command" : "You"}</span>
                  <span className="text-xs text-muted-foreground">{formatTimestamp(message.timestamp)}</span>
                </div>

                <div className="relative">
                  {message.files && message.files.length > 0 && (
                    <div className="mb-3 flex flex-wrap gap-2">
                      {message.files.map((file, index) => (
                        <div
                          key={index}
                          className="flex items-center gap-2 bg-secondary rounded-md px-3 py-1.5 text-sm"
                        >
                          {file.type.startsWith("image/") ? (
                            <ImageIcon className="h-4 w-4" />
                          ) : (
                            <FileText className="h-4 w-4" />
                          )}
                          <span className="truncate max-w-[150px]">{file.name}</span>
                        </div>
                      ))}
                    </div>
                  )}

                  {message.role === "assistant" && message.isCommand && !message.isError ? (
                    <div className="bg-secondary rounded-md overflow-hidden">
                      <div className="bg-secondary/80 px-4 py-2 flex items-center justify-between border-b">
                        <div className="flex items-center gap-2">
                          <Terminal className="h-4 w-4" />
                          <span className="text-sm font-medium">Google Cloud Command</span>
                        </div>
                        <Button
                          variant="ghost"
                          size="sm"
                          className="h-8 px-2"
                          onClick={() => copyToClipboard(message.content, message.id)}
                        >
                          {copiedMessageId === message.id ? (
                            <Check className="h-4 w-4 mr-1" />
                          ) : (
                            <Copy className="h-4 w-4 mr-1" />
                          )}
                          {copiedMessageId === message.id ? "Copied" : "Copy"}
                        </Button>
                      </div>
                      <div className="p-4 font-mono text-sm overflow-x-auto">{message.content}</div>
                    </div>
                  ) : (
                    <div
                      className={cn(
                        "prose prose-sm dark:prose-invert max-w-none markdown",
                        message.isError ? "text-destructive" : "",
                        message.role === "assistant" ? "" : "prose-p:my-1.5",
                      )}
                    >
                      <ReactMarkdown>{message.content}</ReactMarkdown>
                    </div>
                  )}

                  {message.role === "user" && (
                    <Button
                      variant="ghost"
                      size="icon"
                      className={cn("absolute -right-10 top-0 opacity-0 transition-opacity", "group-hover:opacity-100")}
                      onClick={() => copyToClipboard(message.content, message.id)}
                    >
                      {copiedMessageId === message.id ? <Check className="h-4 w-4" /> : <Copy className="h-4 w-4" />}
                    </Button>
                  )}
                </div>
              </div>
            </div>
          ))
        )}

        {isProcessing && (
          <div className="flex gap-3 items-start">
            <Avatar className="h-8 w-8 rounded-md bg-primary/10">
              <Terminal className="h-5 w-5 text-primary" />
            </Avatar>
            <div className="space-y-2">
              <div className="flex items-center gap-2">
                <span className="font-medium">GCloud Command</span>
                <span className="text-xs text-muted-foreground">{formatTimestamp(new Date().toISOString())}</span>
              </div>
              <div className="flex items-center gap-1">
                <div className="h-2 w-2 rounded-full bg-primary animate-bounce" style={{ animationDelay: "0ms" }}></div>
                <div
                  className="h-2 w-2 rounded-full bg-primary animate-bounce"
                  style={{ animationDelay: "150ms" }}
                ></div>
                <div
                  className="h-2 w-2 rounded-full bg-primary animate-bounce"
                  style={{ animationDelay: "300ms" }}
                ></div>
              </div>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>
    </ScrollArea>
  )
}

