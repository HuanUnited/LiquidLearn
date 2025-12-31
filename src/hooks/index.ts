import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import {
  subjectAPI,
  topicAPI,
  theoryAPI,
  fsrsAPI,
  errorAPI,
} from "@/lib/tauri-api";
import toast from "react-hot-toast";

// ============ SUBJECTS ============
export const useSubjects = () => {
  return useQuery({
    queryKey: ["subjects"],
    queryFn: () => subjectAPI.list(),
  });
};

export const useCreateSubject = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ name, description }: { name: string; description?: string }) =>
      subjectAPI.create(name, description),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["subjects"] });
      toast.success("Subject created!");
    },
    onError: () => toast.error("Failed to create subject"),
  });
};

// ============ TOPICS ============
export const useTopics = (subjectId?: string) => {
  return useQuery({
    queryKey: ["topics", subjectId],
    queryFn: () =>
      subjectId ? topicAPI.listBySubject(subjectId) : Promise.resolve([]),
    enabled: !!subjectId,
  });
};

export const useCreateTopic = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      subjectId,
      name,
      description,
    }: {
      subjectId: string;
      name: string;
      description?: string;
    }) => topicAPI.create(subjectId, name, description),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["topics"] });
      toast.success("Topic created!");
    },
    onError: () => toast.error("Failed to create topic"),
  });
};

// ============ THEORIES ============
export const useTheories = (topicId?: string) => {
  return useQuery({
    queryKey: ["theories", topicId],
    queryFn: () =>
      topicId ? theoryAPI.listByTopic(topicId) : Promise.resolve([]),
    enabled: !!topicId,
  });
};

// ============ FSRS ============
export const useFsrsStats = () => {
  return useQuery({
    queryKey: ["fsrs-stats"],
    queryFn: () => fsrsAPI.getStats(),
    refetchInterval: 5000,
  });
};

export const useDueCards = () => {
  return useQuery({
    queryKey: ["fsrs-due"],
    queryFn: () => fsrsAPI.getDueCards(),
  });
};

// ============ ERRORS ============
export const useErrorTypes = () => {
  return useQuery({
    queryKey: ["error-types"],
    queryFn: () => errorAPI.getTypes(),
  });
};

export const useInitErrorTypes = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => errorAPI.init(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["error-types"] });
    },
  });
};

// ============ PERSISTENT LAYOUT ============
export const usePersistentLayout = () => {
  const key = "grid-layout";

  const getLayout = () => {
    const stored = localStorage.getItem(key);
    return stored ? JSON.parse(stored) : null;
  };

  const setLayout = (layout: any) => {
    localStorage.setItem(key, JSON.stringify(layout));
  };

  return { getLayout, setLayout };
};
