export const ssr = true;
import { getRepoStats } from "../server/github.server";

export const load = async () => {
  async function fetchRepo() {
    return await getRepoStats();
  }
 
  return {
    repo: fetchRepo()
  };
};