"use client"

import { useState, useRef, useEffect } from "react"
import { Sidebar } from "@/components/sidebar"
import { ChatHeader } from "@/components/chat-header"
import { ChatInput } from "@/components/chat-input"
import { ChatMessages } from "@/components/chat-messages"
import { MobileNav } from "@/components/mobile-nav"
import { useMediaQuery } from "@/hooks/use-media-query"
import { cn } from "@/lib/utils"
import type { Message, Conversation } from "@/types"
import { v4 as uuidv4 } from "uuid"

// Backend API URL
const API_URL = "http://localhost:5000/process_text"

export function ChatInterface() {
  const [conversations, setConversations] = useState<Conversation[]>([
    { id: "default", title: "New GCloud Command", messages: [] },
  ])
  const [currentConversationId, setCurrentConversationId] = useState<string>("default")
  const [isSidebarOpen, setIsSidebarOpen] = useState(true)
  const [isProcessing, setIsProcessing] = useState(false)
  const messagesEndRef = useRef<HTMLDivElement>(null)
  const isDesktop = useMediaQuery("(min-width: 1024px)")

  // Find current conversation
  const currentConversation = conversations.find((conv) => conv.id === currentConversationId) || conversations[0]

  // Auto-scroll to bottom when messages change
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" })
  }, [currentConversation.messages])

  // Auto-close sidebar on mobile
  useEffect(() => {
    if (!isDesktop) {
      setIsSidebarOpen(false)
    } else {
      setIsSidebarOpen(true)
    }
  }, [isDesktop])

  const handleSendMessage = async (content: string, files?: File[]) => {
    if (!content.trim() && (!files || files.length === 0)) return

    // Create user message
    const userMessage: Message = {
      id: uuidv4(),
      role: "user",
      content,
      timestamp: new Date().toISOString(),
      files: files ? files.map((file) => ({ name: file.name, type: file.type, size: file.size })) : undefined,
    }

    // Update conversation with user message
    const updatedConversations = conversations.map((conv) => {
      if (conv.id === currentConversationId) {
        // Update conversation title if it's the first message
        const title =
          conv.messages.length === 0 ? content.slice(0, 30) + (content.length > 30 ? "..." : "") : conv.title
        return {
          ...conv,
          title,
          messages: [...conv.messages, userMessage],
        }
      }
      return conv
    })

    setConversations(updatedConversations)
    setIsProcessing(true)

    try {
      // Call the Flask backend API
      const response = await fetch(API_URL, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ text: content }),
      })

      if (!response.ok) {
        throw new Error(`API request failed with status ${response.status}`)
      }

      const data = await response.json()
      const gcloudCommand = data.response

      // Create AI response message with the GCloud command
      const aiMessage: Message = {
        id: uuidv4(),
        role: "assistant",
        content: gcloudCommand,
        timestamp: new Date().toISOString(),
        isCommand: true,
      }

      // Update conversations with AI response
      const updatedWithAIResponse = updatedConversations.map((conv) => {
        if (conv.id === currentConversationId) {
          return {
            ...conv,
            messages: [...conv.messages, aiMessage],
          }
        }
        return conv
      })

      setConversations(updatedWithAIResponse)
    } catch (error) {
      console.error("Error calling API:", error)

      // Create error message
      const errorMessage: Message = {
        id: uuidv4(),
        role: "assistant",
        content:
          "Sorry, I couldn't process your request. Please check if the backend server is running at http://localhost:5000.",
        timestamp: new Date().toISOString(),
        isError: true,
      }

      // Update conversations with error message
      const updatedWithError = updatedConversations.map((conv) => {
        if (conv.id === currentConversationId) {
          return {
            ...conv,
            messages: [...conv.messages, errorMessage],
          }
        }
        return conv
      })

      setConversations(updatedWithError)
    } finally {
      setIsProcessing(false)
    }
  }

  const createNewConversation = () => {
    const newConversation: Conversation = {
      id: uuidv4(),
      title: "New GCloud Command",
      messages: [],
    }
    setConversations([...conversations, newConversation])
    setCurrentConversationId(newConversation.id)
  }

  const deleteConversation = (id: string) => {
    const updatedConversations = conversations.filter((conv) => conv.id !== id)
    setConversations(updatedConversations)

    // If we deleted the current conversation, switch to the first available one
    if (id === currentConversationId && updatedConversations.length > 0) {
      setCurrentConversationId(updatedConversations[0].id)
    }

    // If no conversations left, create a new one
    if (updatedConversations.length === 0) {
      createNewConversation()
    }
  }

  const clearConversation = () => {
    const updatedConversations = conversations.map((conv) => {
      if (conv.id === currentConversationId) {
        return {
          ...conv,
          messages: [],
          title: "New GCloud Command",
        }
      }
      return conv
    })
    setConversations(updatedConversations)
  }

  return (
    <div className="flex h-screen overflow-hidden">
      {/* Mobile Navigation */}
      <MobileNav isOpen={isSidebarOpen} onOpenChange={setIsSidebarOpen} onNewChat={createNewConversation} />

      {/* Sidebar */}
      <Sidebar
        conversations={conversations}
        currentConversationId={currentConversationId}
        onSelectConversation={setCurrentConversationId}
        onNewConversation={createNewConversation}
        onDeleteConversation={deleteConversation}
        isOpen={isSidebarOpen}
        onOpenChange={setIsSidebarOpen}
      />

      {/* Main Chat Area */}
      <div
        className={cn(
          "flex flex-col flex-1 h-full transition-all duration-300",
          isSidebarOpen && isDesktop ? "lg:pl-80" : "",
        )}
      >
        <ChatHeader
          title={currentConversation.title}
          onMenuClick={() => setIsSidebarOpen(!isSidebarOpen)}
          onClearChat={clearConversation}
        />

        <div className="flex-1 overflow-hidden relative">
          <ChatMessages
            messages={currentConversation.messages}
            isProcessing={isProcessing}
            messagesEndRef={messagesEndRef}
          />
        </div>

        <ChatInput onSendMessage={handleSendMessage} isDisabled={isProcessing} />
      </div>
    </div>
  )
}

