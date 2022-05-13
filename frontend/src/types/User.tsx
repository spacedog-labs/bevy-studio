export type User = {
  id: string;
  avatar_url: string;
};

export type Project = {
  id: string;
  name: string;
  owner_id: string;
  is_public: boolean;
  entry_point: string;
  release_folder: string;
  is_released: boolean;
  release_id: string;
};
