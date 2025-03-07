"use client"

import type React from "react"
import { useState, useRef, useEffect } from "react"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Send, Paperclip, Mic, X, ImageIcon, FileText, File } from "lucide-react"
import { cn } from "@/lib/utils"

interface ChatInputProps {
  onSendMessage: (content: string, files?: File[]) => void
  isDisabled?: boolean
}

export function ChatInput({ onSendMessage, isDisabled = false }: ChatInputProps) {
  const [message, setMessage] = useState("")
  const [files, setFiles] = useState<File[]>([])
  const [isRecording, setIsRecording] = useState(false)
  const textareaRef = useRef<HTMLTextAreaElement>(null)
  const fileInputRef = useRef<HTMLInputElement>(null)
  const recognitionRef = useRef<SpeechRecognition | null>(null)

  // Initialize Speech Recognition
  useEffect(() => {
    if ("webkitSpeechRecognition" in window) {
      const recognition = new window.webkitSpeechRecognition()
      recognition.continuous = false
      recognition.interimResults = false
      recognition.lang = "en-US"

      recognition.onresult = (event) => {
        const transcript = event.results[0][0].transcript
        setMessage((prev) => prev + (prev ? " " : "") + transcript)
      }

      recognition.onerror = (event) => {
        console.error("Speech Recognition Error:", event)
        setIsRecording(false)
      }

      recognition.onend = () => setIsRecording(false)

      recognitionRef.current = recognition
    }
  }, [])

  // Auto-resize textarea
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto"
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`
    }
  }, [message])

  const handleSend = () => {
    if (isDisabled) return
    if (message.trim() || files.length > 0) {
      onSendMessage(message, files.length > 0 ? files : undefined)
      setMessage("")
      setFiles([])
      if (textareaRef.current) {
        textareaRef.current.style.height = "auto"
      }
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault()
      handleSend()
    }
  }

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      const newFiles = Array.from(e.target.files)
      setFiles([...files, ...newFiles])
    }
  }

  const removeFile = (index: number) => {
    setFiles(files.filter((_, i) => i !== index))
  }

  const getFileIcon = (file: File) => {
    if (file.type.startsWith("image/")) {
      return <ImageIcon className="h-4 w-4" />
    } else if (file.type.includes("pdf") || file.type.includes("document")) {
      return <FileText className="h-4 w-4" />
    } else {
      return <File className="h-4 w-4" />
    }
  }

  const toggleRecording = () => {
    if (!recognitionRef.current) return

    if (isRecording) {
      recognitionRef.current.stop()
    } else {
      setMessage("")
      recognitionRef.current.start()
    }
    setIsRecording(!isRecording)
  }

  return (
    <div className="border-t bg-background p-4">
      {files.length > 0 && (
        <div className="mb-3 flex flex-wrap gap-2">
          {files.map((file, index) => (
            <div key={index} className="flex items-center gap-2 bg-secondary rounded-md px-3 py-1.5 text-sm">
              {getFileIcon(file)}
              <span className="truncate max-w-[150px]">{file.name}</span>
              <Button variant="ghost" size="icon" className="h-5 w-5" onClick={() => removeFile(index)}>
                <X className="h-3 w-3" />
              </Button>
            </div>
          ))}
        </div>
      )}

      <div className="relative flex items-end gap-2">
        <Button
          variant="outline"
          size="icon"
          className="shrink-0"
          onClick={() => fileInputRef.current?.click()}
          disabled={isDisabled}
        >
          <Paperclip className="h-5 w-5" />
          <input type="file" ref={fileInputRef} className="hidden" multiple onChange={handleFileChange} />
        </Button>

        <div className="relative flex-1">
          <Textarea
            ref={textareaRef}
            placeholder="Describe what you want to do in Google Cloud..."
            value={message}
            onChange={(e) => setMessage(e.target.value)}
            onKeyDown={handleKeyDown}
            className={cn(
              "min-h-[60px] w-full resize-none rounded-md py-3 pr-12",
              "scrollbar-w-2 scrollbar-track-transparent scrollbar-thumb-rounded scrollbar-thumb-gray",
            )}
            disabled={isDisabled}
          />
          <Button
            variant="ghost"
            size="icon"
            className="absolute bottom-2 right-2"
            onClick={handleSend}
            disabled={isDisabled || (message.trim() === "" && files.length === 0)}
          >
            <Send className="h-5 w-5" />
          </Button>
        </div>

        <Button
          variant={isRecording ? "destructive" : "outline"}
          size="icon"
          className="shrink-0"
          onClick={toggleRecording}
          disabled={isDisabled}
        >
          <Mic className="h-5 w-5" />
        </Button>
      </div>

      <div className="mt-2 text-xs text-center text-muted-foreground">
        Type your request in natural language or use voice input to get the equivalent Google Cloud command.
      </div>
    </div>
  )
}
