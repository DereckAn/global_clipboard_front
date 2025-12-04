export type LabFeatureId = "screenshot" | "ocr" | "translator" | "pickColor";

export interface LabFeatureMeta {
  id: LabFeatureId;
  title: string;
  description: string;
  icon: string;
  needsDownload: boolean;
}

export interface LabFeatureState {
  id: LabFeatureId;
  status: "available" | "installed";
  progress?: number;
  error?: string | null;
  enabled: boolean;
  installedVersion?: string;
}

export interface LabFeatureWithMeta extends LabFeatureMeta, LabFeatureState {}
