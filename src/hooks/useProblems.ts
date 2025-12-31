import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { problemAPI, attemptAPI, fsrsAPI, errorAPI } from "@/lib/tauri-api";
import type { Problem, Attempt } from "@/types";
import toast from "react-hot-toast";

export const useProblems = (topicId?: string) => {
  return useQuery({
    queryKey: ["problems", topicId],
    queryFn: () =>
      topicId ? problemAPI.listByTopic(topicId) : Promise.resolve([]),
    enabled: !!topicId,
  });
};

export const useProblem = (problemId: string) => {
  return useQuery({
    queryKey: ["problem", problemId],
    queryFn: () => problemAPI.getWithDetails(problemId),
  });
};

export const useCreateProblem = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      topicId,
      title,
      description,
      difficulty,
    }: {
      topicId: string;
      title: string;
      description?: string;
      difficulty?: number;
    }) =>
      problemAPI.create(topicId, title, description, undefined, difficulty),
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: ["problems"] });
      toast.success("Problem created!");
    },
    onError: (error: any) => {
      toast.error(error.message || "Failed to create problem");
    },
  });
};

export const useSubmitAttempt = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({
      problemId,
      isSolved,
      commentary,
      errors,
      quality,
      timeSpent,
    }: {
      problemId: string;
      isSolved: boolean;
      commentary?: string;
      errors?: Array<{ error_type_id: number; description?: string }>;
      quality?: number;
      timeSpent?: number;
    }) => {
      // Create attempt
      const attempt = await attemptAPI.create(problemId, isSolved, commentary);

      // Log errors if not solved
      if (!isSolved && errors) {
        for (const error of errors) {
          await errorAPI.log(
            attempt.id,
            error.error_type_id,
            error.description
          );
        }
      }

      // Process FSRS review
      if (quality !== undefined && timeSpent !== undefined) {
        await fsrsAPI.processReview(problemId, isSolved, quality, timeSpent);
      }

      return attempt;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["problems"] });
      queryClient.invalidateQueries({ queryKey: ["fsrs-stats"] });
      toast.success("Attempt recorded!");
    },
    onError: (error: any) => {
      toast.error(error.message || "Failed to submit attempt");
    },
  });
};
