//File with 400 lines of code

// Enums
enum TaskStatus {
  TODO = 'TODO',
  IN_PROGRESS = 'IN_PROGRESS',
  REVIEW = 'REVIEW',
  DONE = 'DONE',
}

enum TaskPriority {
  LOW = 'LOW',
  MEDIUM = 'MEDIUM',
  HIGH = 'HIGH',
  URGENT = 'URGENT',
}

enum UserRole {
  DEVELOPER = 'DEVELOPER',
  DESIGNER = 'DESIGNER',
  MANAGER = 'MANAGER',
  ADMIN = 'ADMIN',
}

// Interfaces
interface ITask {
  id: string;
  title: string;
  description: string;
  status: TaskStatus;
  priority: TaskPriority;
  dueDate: Date;
  assignedTo: string | null;
  createdAt: Date;
  updatedAt: Date;
  comments: IComment[];
}

interface IProject {
  id: string;
  name: string;
  description: string;
  tasks: Task[];
  createdAt: Date;
  updatedAt: Date;
  owner: string;
}

interface IUser {
  id: string;
  name: string;
  email: string;
  role: UserRole;
  createdAt: Date;
  updatedAt: Date;
}

interface IComment {
  id: string;
  content: string;
  author: string;
  createdAt: Date;
}

// Custom Error Classes
class TaskManagementError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'TaskManagementError';
  }
}

class TaskNotFoundError extends TaskManagementError {
  constructor(taskId: string) {
    super(`Task with id ${taskId} not found`);
    this.name = 'TaskNotFoundError';
  }
}

class UserNotFoundError extends TaskManagementError {
  constructor(userId: string) {
    super(`User with id ${userId} not found`);
    this.name = 'UserNotFoundError';
  }
}

class ProjectNotFoundError extends TaskManagementError {
  constructor(projectId: string) {
    super(`Project with id ${projectId} not found`);
    this.name = 'ProjectNotFoundError';
  }
}

// Classes
class Task implements ITask {
  public comments: Comment[] = [];

  constructor(
    public id: string,
    public title: string,
    public description: string,
    public status: TaskStatus = TaskStatus.TODO,
    public priority: TaskPriority = TaskPriority.MEDIUM,
    public dueDate: Date = new Date(),
    public assignedTo: string | null = null,
    public createdAt: Date = new Date(),
    public updatedAt: Date = new Date()
  ) {}

  updateStatus(newStatus: TaskStatus): void {
    this.status = newStatus;
    this.updatedAt = new Date();
  }

  updatePriority(newPriority: TaskPriority): void {
    this.priority = newPriority;
    this.updatedAt = new Date();
  }

  assignTo(userId: string): void {
    this.assignedTo = userId;
    this.updatedAt = new Date();
  }

  isOverdue(): boolean {
    return new Date() > this.dueDate;
  }

  addComment(comment: Comment): void {
    this.comments.push(comment);
    this.updatedAt = new Date();
  }
}

class Project implements IProject {
  public createdAt: Date;
  public updatedAt: Date;

  constructor(
    public id: string,
    public name: string,
    public description: string,
    public owner: string,
    public tasks: Task[] = []
  ) {
    this.createdAt = new Date();
    this.updatedAt = new Date();
  }

  addTask(task: Task): void {
    this.tasks.push(task);
    this.updatedAt = new Date();
  }

  removeTask(taskId: string): void {
    this.tasks = this.tasks.filter(task => task.id !== taskId);
    this.updatedAt = new Date();
  }

  getTaskById(taskId: string): Task | undefined {
    return this.tasks.find(task => task.id === taskId);
  }

  getTasksByStatus(status: TaskStatus): Task[] {
    return this.tasks.filter(task => task.status === status);
  }

  getOverdueTasks(): Task[] {
    return this.tasks.filter(task => task.isOverdue());
  }

  updateName(newName: string): void {
    this.name = newName;
    this.updatedAt = new Date();
  }

  updateDescription(newDescription: string): void {
    this.description = newDescription;
    this.updatedAt = new Date();
  }
}

class User implements IUser {
  public createdAt: Date;
  public updatedAt: Date;

  constructor(
    public id: string,
    public name: string,
    public email: string,
    public role: UserRole
  ) {
    this.createdAt = new Date();
    this.updatedAt = new Date();
  }

  updateRole(newRole: UserRole): void {
    this.role = newRole;
    this.updatedAt = new Date();
  }

  updateName(newName: string): void {
    this.name = newName;
    this.updatedAt = new Date();
  }

  updateEmail(newEmail: string): void {
    this.email = newEmail;
    this.updatedAt = new Date();
  }
}

class Comment implements IComment {
  constructor(
    public id: string,
    public content: string,
    public author: string,
    public createdAt: Date = new Date()
  ) {}
}

// Utility functions
function generateId(): string {
  return Math.random().toString(36).substr(2, 9);
}

function createTask(
  title: string,
  description: string,
  priority: TaskPriority,
  dueDate: Date
): Task {
  return new Task(generateId(), title, description, TaskStatus.TODO, priority, dueDate);
}

function createProject(name: string, description: string, ownerId: string): Project {
  return new Project(generateId(), name, description, ownerId);
}

function createUser(name: string, email: string, role: UserRole): User {
  return new User(generateId(), name, email, role);
}

function createComment(content: string, authorId: string): Comment {
  return new Comment(generateId(), content, authorId);
}

// Task Management System
class TaskManagementSystem {
  private projects: Project[] = [];
  private users: User[] = [];

  addProject(project: Project): void {
    this.projects.push(project);
  }

  addUser(user: User): void {
    this.users.push(user);
  }

  getProjectById(projectId: string): Project {
    const project = this.projects.find(p => p.id === projectId);
    if (!project) {
      throw new ProjectNotFoundError(projectId);
    }
    return project;
  }

  getUserById(userId: string): User {
    const user = this.users.find(u => u.id === userId);
    if (!user) {
      throw new UserNotFoundError(userId);
    }
    return user;
  }

  assignTaskToUser(taskId: string, userId: string): void {
    const user = this.getUserById(userId);

    for (const project of this.projects) {
      const task = project.getTaskById(taskId);
      if (task) {
        task.assignTo(userId);
        return;
      }
    }

    throw new TaskNotFoundError(taskId);
  }

  addCommentToTask(taskId: string, comment: Comment): void {
    for (const project of this.projects) {
      const task = project.getTaskById(taskId);
      if (task) {
        task.addComment(comment);
        return;
      }
    }

    throw new TaskNotFoundError(taskId);
  }

  getProjectStats(projectId: string): { total: number, todo: number, inProgress: number, review: number, done: number } {
    const project = this.getProjectById(projectId);

    return {
      total: project.tasks.length,
      todo: project.getTasksByStatus(TaskStatus.TODO).length,
      inProgress: project.getTasksByStatus(TaskStatus.IN_PROGRESS).length,
      review: project.getTasksByStatus(TaskStatus.REVIEW).length,
      done: project.getTasksByStatus(TaskStatus.DONE).length,
    };
  }

  getOverdueTasksForUser(userId: string): Task[] {
    this.getUserById(userId); // Verify user exists
    const overdueTasks: Task[] = [];

    for (const project of this.projects) {
      const userOverdueTasks = project.getOverdueTasks().filter(task => task.assignedTo === userId);
      overdueTasks.push(...userOverdueTasks);
    }

    return overdueTasks;
  }

  getUserProjects(userId: string): Project[] {
    this.getUserById(userId); // Verify user exists
    return this.projects.filter(project => project.owner === userId);
  }

  getProjectTasks(projectId: string): Task[] {
    const project = this.getProjectById(projectId);
    return project.tasks;
  }

  updateTaskStatus(taskId: string, newStatus: TaskStatus): void {
    for (const project of this.projects) {
      const task = project.getTaskById(taskId);
      if (task) {
        task.updateStatus(newStatus);
        return;
      }
    }

    throw new TaskNotFoundError(taskId);
  }

  updateTaskPriority(taskId: string, newPriority: TaskPriority): void {
    for (const project of this.projects) {
      const task = project.getTaskById(taskId);
      if (task) {
        task.updatePriority(newPriority);
        return;
      }
    }

    throw new TaskNotFoundError(taskId);
  }

  deleteProject(projectId: string): void {
    const index = this.projects.findIndex(p => p.id === projectId);
    if (index === -1) {
      throw new ProjectNotFoundError(projectId);
    }
    this.projects.splice(index, 1);
  }

  deleteUser(userId: string): void {
    const index = this.users.findIndex(u => u.id === userId);
    if (index === -1) {
      throw new UserNotFoundError(userId);
    }
    this.users.splice(index, 1);
  }
}

// Usage example
const taskManagementSystem = new TaskManagementSystem();

try {
  const user1 = createUser('Alice', 'alice@example.com', UserRole.DEVELOPER);
  const user2 = createUser('Bob', 'bob@example.com', UserRole.DESIGNER);
  const user3 = createUser('Charlie', 'charlie@example.com', UserRole.MANAGER);

  taskManagementSystem.addUser(user1);
  taskManagementSystem.addUser(user2);
  taskManagementSystem.addUser(user3);

  const project1 = createProject('Website Redesign', 'Redesign company website', user3.id);
  const project2 = createProject('Mobile App Development', 'Develop a new mobile app', user3.id);

  taskManagementSystem.addProject(project1);
  taskManagementSystem.addProject(project2);

  const task1 = createTask('Design homepage', 'Create new homepage design', TaskPriority.HIGH, new Date('2023-12-31'));
  const task2 = createTask('Implement login', 'Add user authentication', TaskPriority.MEDIUM, new Date('2024-01-15'));

  project1.addTask(task1);
  project1.addTask(task2);

  taskManagementSystem.assignTaskToUser(task1.id, user2.id);
  taskManagementSystem.assignTaskToUser(task2.id, user1.id);

  const comment1 = createComment('Initial design draft completed', user2.id);
  taskManagementSystem.addCommentToTask(task1.id, comment1);

  console.log('Project stats:', taskManagementSystem.getProjectStats(project1.id));
  console.log('Overdue tasks for Alice:', taskManagementSystem.getOverdueTasksForUser(user1.id));
  console.log('Charlie\'s projects:', taskManagementSystem.getUserProjects(user3.id));

  taskManagementSystem.updateTaskStatus(task1.id, TaskStatus.IN_PROGRESS);
  taskManagementSystem.updateTaskPriority(task2.id, TaskPriority.HIGH);

  console.log('Updated project tasks:', taskManagementSystem.getProjectTasks(project1.id));

} catch (error) {
  if (error instanceof TaskManagementError) {
    console.error('Task Management Error:', error.message);
  } else {
    console.error('Unexpected error:', error);
  }
}