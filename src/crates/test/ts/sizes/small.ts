// types.ts - This file handles the types and interfaces used in the application

// Defining a few interfaces for the project

export interface User {
  id: number;
  username: string;
  email: string;
  age: number;
  active: boolean;
}

export interface Article {
  id: number;
  title: string;
  content: string;
  author: User;
  publishedAt: Date;
}

export interface Comment {
  id: number;
  author: User;
  content: string;
  createdAt: Date;
  articleId: number;
}

// Defining Enums for some status codes
export enum StatusCode {
  SUCCESS = 200,
  NOT_FOUND = 404,
  SERVER_ERROR = 500,
}

export enum Role {
  ADMIN = "admin",
  EDITOR = "editor",
  USER = "user",
}

// Generics
export function filterActiveUsers<T extends User>(users: T[]): T[] {
  return users.filter((user) => user.active);
}

// -------------------------------------------------------
// services.ts - Service layer handles fetching data from APIs

// Simulating a basic API call using promises
export function fetchUserById(id: number): Promise<User | null> {
  return new Promise((resolve) => {
    setTimeout(() => {
      if (id > 1000) {
        resolve(null);
      } else {
        resolve({
          id,
          username: `User_${id}`,
          email: `user${id}@mail.com`,
          age: 25,
          active: true,
        });
      }
    }, 500);
  });
}

export function fetchArticles(): Promise<Article[]> {
  return new Promise((resolve) => {
    const articles: Article[] = [
      {
        id: 1,
        title: "TypeScript for Beginners",
        content: "This is a great introduction to TypeScript.",
        author: {
          id: 1,
          username: "author1",
          email: "author1@mail.com",
          age: 30,
          active: true,
        },
        publishedAt: new Date(),
      },
      {
        id: 2,
        title: "Advanced TypeScript",
        content: "Advanced concepts of TypeScript explained.",
        author: {
          id: 2,
          username: "author2",
          email: "author2@mail.com",
          age: 32,
          active: true,
        },
        publishedAt: new Date(),
      },
    ];
    resolve(articles);
  });
}

export function fetchCommentsByArticleId(
  articleId: number,
): Promise<Comment[]> {
  return new Promise((resolve) => {
    const comments: Comment[] = [
      {
        id: 1,
        author: {
          id: 1,
          username: "commenter1",
          email: "commenter1@mail.com",
          age: 27,
          active: true,
        },
        content: "Great article!",
        createdAt: new Date(),
        articleId,
      },
      {
        id: 2,
        author: {
          id: 2,
          username: "commenter2",
          email: "commenter2@mail.com",
          age: 29,
          active: true,
        },
        content: "Very informative.",
        createdAt: new Date(),
        articleId,
      },
    ];
    resolve(comments);
  });
}

// -------------------------------------------------------
// utils.ts - Helper utilities for the application

export function formatDate(date: Date): string {
  return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;
}

export function capitalize(text: string): string {
  return text.charAt(0).toUpperCase() + text.slice(1);
}

export function generateRandomId(): number {
  return Math.floor(Math.random() * 10000);
}

// -------------------------------------------------------
// main.ts - Entry point for the application, handles logic

// Sample asynchronous function to load data
async function loadUserData(userId: number) {
  const user = await fetchUserById(userId);
  if (user) {
    console.log(
      `User: ${capitalize(user.username)}, Email: ${user.email}, Age: ${user.age}`,
    );
  } else {
    console.log("User not found.");
  }
}

async function loadArticles() {
  const articles = await fetchArticles();
  articles.forEach((article) => {
    console.log(`Title: ${article.title}`);
    console.log(`Published: ${formatDate(article.publishedAt)}`);
  });
}

async function loadCommentsForArticle(articleId: number) {
  const comments = await fetchCommentsByArticleId(articleId);
  comments.forEach((comment) => {
    console.log(
      `Comment by: ${capitalize(comment.author.username)} - ${comment.content}`,
    );
  });
}

// Run the functions
loadUserData(1);
loadArticles();
loadCommentsForArticle(1);

// -------------------------------------------------------
// event_manager.ts - Handling event-driven programming

export type EventCallback = () => void;

export class EventManager {
  private events: { [key: string]: EventCallback[] } = {};

  public subscribe(event: string, callback: EventCallback) {
    if (!this.events[event]) {
      this.events[event] = [];
    }
    this.events[event].push(callback);
  }

  public publish(event: string) {
    if (this.events[event]) {
      this.events[event].forEach((callback) => callback());
    }
  }

  public unsubscribe(event: string, callback: EventCallback) {
    if (!this.events[event]) return;
    this.events[event] = this.events[event].filter((cb) => cb !== callback);
  }
}

// -------------------------------------------------------
// app.ts - Application initialization and event handling

const eventManager = new EventManager();

// Example of subscribing to an event
eventManager.subscribe("userLogin", () => {
  console.log("User logged in!");
});

// Example of publishing an event
eventManager.publish("userLogin");

// Example of unsubscribing from an event
eventManager.unsubscribe("userLogin", () => {
  console.log("User logged in!");
});
