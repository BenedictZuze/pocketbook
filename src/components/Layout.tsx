import React from "react";
import { Link, useLocation } from "wouter";
import { Database, Plus } from "lucide-react";

interface LayoutProps {
  children: React.ReactNode;
}

export const Layout: React.FC<LayoutProps> = ({ children }) => {
  const [location] = useLocation();

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <Link href="/">
              <div className="flex items-center space-x-3 cursor-pointer group">
                <Database className="h-8 w-8 text-blue-600 group-hover:text-blue-700 transition-colors" />
                <h1 className="text-xl font-bold text-gray-900 group-hover:text-blue-700 transition-colors">
                  PocketBase Manager
                </h1>
              </div>
            </Link>

            <nav className="flex items-center space-x-4">
              <Link href="/">
                <a
                  className={`px-3 py-2 rounded-md text-sm font-medium transition-colors ${
                    location === "/"
                      ? "bg-blue-100 text-blue-700"
                      : "text-gray-600 hover:text-gray-900 hover:bg-gray-100"
                  }`}
                >
                  Projects
                </a>
              </Link>
              <Link href="/new">
                <a className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors">
                  <Plus className="h-4 w-4 mr-2" />
                  New Project
                </a>
              </Link>
            </nav>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {children}
      </main>
    </div>
  );
};
